#!/usr/bin/env python3
"""Categorise TMDb cartoon series with a local Ollama model.

The LLM categorises each series once. Episode data is only used as bounded
context, capped by the `episodes` value from the JSON config.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sqlite3
import sys
import time
from pathlib import Path
from typing import Any
from urllib import error, parse, request


TMDB_BASE_URL = "https://api.themoviedb.org/3"
OLLAMA_DEFAULT_URL = "http://localhost:11434"


class TerminalUI:
    def __init__(self) -> None:
        self.use_color = sys.stdout.isatty() and os.environ.get("NO_COLOR") is None

    def color(self, value: str, code: str) -> str:
        if not self.use_color:
            return value
        return f"\033[{code}m{value}\033[0m"

    def header(self, title: str) -> None:
        line = "=" * max(24, len(title) + 4)
        print(self.color(line, "36"))
        print(self.color(f"  {title}", "1;36"))
        print(self.color(line, "36"))

    def kv(self, key: str, value: Any) -> None:
        print(f"{self.color(key + ':', '2')} {value}")

    def step(self, index: int, total: int, title: str) -> None:
        print()
        print(self.color(f"[{index}/{total}] {title}", "1;34"))

    def info(self, message: str) -> None:
        print(f"  {self.color('>', '2')} {message}", flush=True)

    def ok(self, message: str) -> None:
        print(f"  {self.color('OK', '32')} {message}", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Search TMDb series ids or categorise configured series with Ollama."
    )
    parser.add_argument("--config", default="scripts/tmdb_categories.example.json")
    parser.add_argument("--db", default=os.environ.get("DATABASE_URL", "sqlite://data/kroissant.sqlite"))
    parser.add_argument("--search", help="Search an exact TV series name in TMDb and print candidate ids.")
    parser.add_argument("--language", default=None, help="TMDb language, e.g. fr-FR.")
    parser.add_argument("--ollama-url", default=None, help="Ollama base URL, e.g. http://localhost:11434.")
    parser.add_argument("--model", default=None, help="Ollama model name. Overrides config and OLLAMA_MODEL.")
    parser.add_argument("--dry-run", action="store_true", help="Fetch TMDb context but do not call Ollama or write SQLite.")
    args = parser.parse_args()
    ui = TerminalUI()
    load_dotenv(Path(".env"))

    if args.search:
        language = args.language or "fr-FR"
        rows = search_tmdb(args.search, language)
        ui.header("Recherche TMDb")
        ui.kv("requete", args.search)
        ui.kv("langue", language)
        print()
        print_search_rows(rows)
        return 0

    config = load_config(Path(args.config))
    language = args.language or config.get("tmdb_language") or "fr-FR"
    ollama_url = args.ollama_url or os.environ.get("OLLAMA_URL") or config.get("ollama_url") or OLLAMA_DEFAULT_URL
    model = args.model or os.environ.get("OLLAMA_MODEL") or config.get("ollama_model") or "llama3.1"
    categories = normalize_categories(config.get("categories", []))
    series_configs = config.get("series", [])

    if not categories:
        raise SystemExit("config.categories must contain at least one tag")
    if not isinstance(series_configs, list) or not series_configs:
        raise SystemExit("config.series must contain at least one series entry")

    ui.header("Categorisation TMDb + Ollama")
    ui.kv("config", args.config)
    ui.kv("base", args.db)
    ui.kv("langue TMDb", language)
    ui.kv("ollama", ollama_url)
    ui.kv("modele", model)
    ui.kv("tags autorises", ", ".join(categories))
    ui.kv("dry run", "oui" if args.dry_run else "non")

    conn = None if args.dry_run else connect_db(args.db)
    if conn is not None:
        ensure_schema(conn)

    for index, item in enumerate(series_configs, start=1):
        tmdb_id = item.get("tmdb_id", item.get("series_id"))
        if tmdb_id is None:
            raise SystemExit(f"missing tmdb_id in series entry: {item}")
        tmdb_id = int(tmdb_id)
        episode_limit = int(item.get("episodes", item.get("episode_count", 5)))
        if episode_limit < 1:
            raise SystemExit(f"episodes must be >= 1 for TMDb id {tmdb_id}")

        ui.step(index, len(series_configs), f"TMDb {tmdb_id}")
        ui.info("recuperation des metadonnees de serie")
        details = tmdb_get(f"/tv/{tmdb_id}", {"language": language})
        title = details.get("name") or details.get("original_name") or str(tmdb_id)
        ui.ok(f"serie trouvee: {title}")
        ui.info(f"recuperation de {episode_limit} episode(s) maximum pour le contexte")
        episodes = collect_episode_context(tmdb_id, details, item, episode_limit, language)
        ui.ok(f"{len(episodes)} episode(s) ajoutes au contexte LLM")
        context = build_series_context(details, episodes, item, episode_limit)

        if args.dry_run:
            ui.info("dry-run: contexte affiche, aucun appel Ollama ni ecriture SQLite")
            print(json.dumps({"tmdb_id": tmdb_id, "context": context}, ensure_ascii=False, indent=2))
            continue

        ui.info("appel Ollama pour categoriser la serie")
        classification = classify_with_ollama(ollama_url, model, categories, context)
        ui.ok(
            f"tags: {', '.join(classification['tags'])} "
            f"(confiance {classification['confidence']:.0%})"
        )
        ui.info("ecriture SQLite des metadonnees, episodes de contexte et tags")
        write_series(conn, details, episodes, item, classification)
        ui.ok("serie enregistree")

    if conn is not None:
        conn.commit()
        conn.close()
    return 0


def print_search_rows(rows: list[dict[str, Any]]) -> None:
    if not rows:
        print("Aucun resultat.")
        return

    print(f"{'id':<9} {'exact':<7} {'date':<12} nom")
    print("-" * 72)
    for row in rows:
        exact = "oui" if row["exact_match"] else "non"
        date = row.get("first_air_date") or "-"
        name = row.get("name") or "-"
        original = row.get("original_name")
        if original and original != name:
            name = f"{name} ({original})"
        print(f"{row.get('id')!s:<9} {exact:<7} {date:<12} {name}")


def load_config(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def load_dotenv(path: Path) -> None:
    if not path.exists():
        return

    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        if line.startswith("export "):
            line = line.removeprefix("export ").strip()
        key, value = line.split("=", 1)
        key = key.strip()
        value = value.strip().strip('"').strip("'")
        if key and key not in os.environ:
            os.environ[key] = value


def normalize_categories(values: list[Any]) -> list[str]:
    seen: set[str] = set()
    categories: list[str] = []
    for value in values:
        tag = normalize_tag(str(value))
        if tag and tag not in seen:
            seen.add(tag)
            categories.append(tag)
    return categories


def normalize_tag(value: str) -> str:
    return re.sub(r"\s+", " ", value.strip().lower())


def normalize_title(value: str) -> str:
    value = value.casefold().strip()
    value = re.sub(r"[^\w\s]", " ", value, flags=re.UNICODE)
    return re.sub(r"\s+", " ", value)


def search_tmdb(query: str, language: str) -> list[dict[str, Any]]:
    target = normalize_title(query)
    data = tmdb_get(
        "/search/tv",
        {
            "query": query,
            "language": language,
            "include_adult": "false",
            "page": 1,
        },
    )
    rows = []
    for item in data.get("results", []):
        names = [
            normalize_title(str(item.get("name", ""))),
            normalize_title(str(item.get("original_name", ""))),
        ]
        exact = target in names
        rows.append(
            {
                "id": item.get("id"),
                "name": item.get("name"),
                "original_name": item.get("original_name"),
                "first_air_date": item.get("first_air_date"),
                "origin_country": item.get("origin_country"),
                "overview": item.get("overview"),
                "exact_match": exact,
            }
        )
    rows.sort(key=lambda row: (not row["exact_match"], row["name"] or ""))
    return rows


def tmdb_get(path: str, params: dict[str, Any]) -> dict[str, Any]:
    token = os.environ.get("TMDB_BEARER_TOKEN")
    api_key = os.environ.get("TMDB_API_KEY")
    if not token and not api_key:
        raise SystemExit("Set TMDB_BEARER_TOKEN or TMDB_API_KEY before calling TMDb.")

    headers = {"Accept": "application/json"}
    query = dict(params)
    if token:
        headers["Authorization"] = f"Bearer {token}"
    else:
        query["api_key"] = api_key

    url = f"{TMDB_BASE_URL}{path}?{parse.urlencode(query)}"
    return http_json("GET", url, headers=headers)


def collect_episode_context(
    tmdb_id: int,
    details: dict[str, Any],
    config: dict[str, Any],
    episode_limit: int,
    language: str,
) -> list[dict[str, Any]]:
    fixed_season = config.get("season")
    start_season = int(config.get("start_season", fixed_season or 1))
    start_episode = int(config.get("start_episode", 1))
    season_numbers = [
        int(season["season_number"])
        for season in details.get("seasons", [])
        if int(season.get("season_number", 0)) > 0
    ]

    if fixed_season is not None:
        season_numbers = [int(fixed_season)]
    else:
        season_numbers = [number for number in season_numbers if number >= start_season]
        if not season_numbers:
            season_numbers = [start_season]

    episodes: list[dict[str, Any]] = []
    for season_number in season_numbers:
        season = tmdb_get(f"/tv/{tmdb_id}/season/{season_number}", {"language": language})
        for episode in season.get("episodes", []):
            episode_number = int(episode.get("episode_number") or 0)
            if season_number == start_season and episode_number < start_episode:
                continue
            episodes.append(
                {
                    "tmdb_episode_id": episode.get("id"),
                    "season_number": season_number,
                    "episode_number": episode_number,
                    "title": episode.get("name") or f"Episode {episode_number}",
                    "overview": episode.get("overview") or "",
                    "air_date": episode.get("air_date"),
                    "runtime": episode.get("runtime"),
                    "still_path": episode.get("still_path"),
                }
            )
            if len(episodes) >= episode_limit:
                return episodes
    return episodes


def build_series_context(
    details: dict[str, Any],
    episodes: list[dict[str, Any]],
    config: dict[str, Any],
    episode_limit: int,
) -> dict[str, Any]:
    return {
        "tmdb_id": details.get("id"),
        "title": details.get("name") or config.get("name") or details.get("original_name"),
        "original_title": details.get("original_name"),
        "overview": truncate(details.get("overview") or "", 1000),
        "first_air_date": details.get("first_air_date"),
        "episode_context_limit": episode_limit,
        "episode_context_used": len(episodes),
        "episodes": [
            {
                "season": episode["season_number"],
                "episode": episode["episode_number"],
                "title": truncate(episode["title"], 180),
                "overview": truncate(episode["overview"], 700),
            }
            for episode in episodes
        ],
    }


def classify_with_ollama(
    ollama_url: str,
    model: str,
    categories: list[str],
    context: dict[str, Any],
) -> dict[str, Any]:
    system_prompt = (
        "Tu categorises une serie de dessin anime pour enfants, au niveau serie uniquement. "
        "Tu utilises seulement le contexte fourni: synopsis de serie et nombre limite d'episodes. "
        "Tu ne categorises pas chaque episode. "
        "Choisis uniquement des tags dans cette liste: "
        f"{', '.join(categories)}. "
        "Reponds en JSON strict avec tags, confidence et reason."
    )
    user_prompt = json.dumps(context, ensure_ascii=False, indent=2)
    schema = {
        "type": "object",
        "additionalProperties": False,
        "properties": {
            "tags": {"type": "array", "items": {"type": "string", "enum": categories}},
            "confidence": {"type": "number"},
            "reason": {"type": "string"},
        },
        "required": ["tags", "confidence", "reason"],
    }
    body = {
        "model": model,
        "stream": False,
        "format": schema,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt},
        ],
        "options": {"temperature": 0},
    }
    url = f"{ollama_url.rstrip('/')}/api/chat"

    try:
        data = http_json("POST", url, body=body)
    except error.HTTPError as exc:
        error_body = read_http_error(exc)
        if exc.code != 400:
            raise RuntimeError(
                f"Ollama a repondu HTTP {exc.code} pour le modele {model}. "
                f"Verifiez `ollama ps` ou lancez `ollama pull {model}`. "
                f"Reponse: {error_body}"
            ) from exc
        body["format"] = "json"
        try:
            data = http_json("POST", url, body=body)
        except error.HTTPError as retry_exc:
            retry_body = read_http_error(retry_exc)
            raise RuntimeError(
                f"Ollama a repondu HTTP {retry_exc.code} pour le modele {model}. "
                f"Verifiez `ollama ps` ou lancez `ollama pull {model}`. "
                f"Reponse: {retry_body}"
            ) from retry_exc

    content = data.get("message", {}).get("content", "")
    parsed = json.loads(extract_json_object(content))
    tags = [normalize_tag(str(tag)) for tag in parsed.get("tags", [])]
    allowed = set(categories)
    tags = [tag for tag in tags if tag in allowed]
    if not tags:
        raise RuntimeError(f"Ollama returned no valid tag for {context.get('title')}: {content}")

    confidence = float(parsed.get("confidence", 0.0))
    if confidence > 1.0 and confidence <= 100.0:
        confidence = confidence / 100.0
    confidence = max(0.0, min(1.0, confidence))

    return {
        "tags": sorted(set(tags), key=tags.index),
        "confidence": confidence,
        "reason": truncate(str(parsed.get("reason", "")), 500),
    }


def extract_json_object(value: str) -> str:
    value = value.strip()
    if value.startswith("```"):
        value = re.sub(r"^```(?:json)?", "", value).strip()
        value = re.sub(r"```$", "", value).strip()
    start = value.find("{")
    end = value.rfind("}")
    if start == -1 or end == -1 or end < start:
        raise ValueError(f"expected JSON object, got: {value}")
    return value[start : end + 1]


def write_series(
    conn: sqlite3.Connection,
    details: dict[str, Any],
    episodes: list[dict[str, Any]],
    config: dict[str, Any],
    classification: dict[str, Any],
) -> None:
    now = iso_now()
    tmdb_id = int(details["id"])
    source_url = f"https://www.themoviedb.org/tv/{tmdb_id}"
    conn.execute(
        """
        INSERT INTO tmdb_series (
            tmdb_id, name, original_name, overview, first_air_date, poster_path,
            platform, age_range, episode_context_count, llm_reason, confidence,
            source_url, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(tmdb_id) DO UPDATE SET
            name = excluded.name,
            original_name = excluded.original_name,
            overview = excluded.overview,
            first_air_date = excluded.first_air_date,
            poster_path = excluded.poster_path,
            platform = excluded.platform,
            age_range = excluded.age_range,
            episode_context_count = excluded.episode_context_count,
            llm_reason = excluded.llm_reason,
            confidence = excluded.confidence,
            source_url = excluded.source_url,
            updated_at = excluded.updated_at
        """,
        (
            tmdb_id,
            details.get("name") or config.get("name") or details.get("original_name") or "",
            details.get("original_name") or "",
            details.get("overview") or "",
            details.get("first_air_date"),
            details.get("poster_path"),
            config.get("platform", ""),
            config.get("age_range", ""),
            len(episodes),
            classification["reason"],
            classification["confidence"],
            source_url,
            now,
        ),
    )
    series_id = conn.execute("SELECT id FROM tmdb_series WHERE tmdb_id = ?", (tmdb_id,)).fetchone()[0]

    conn.execute("DELETE FROM tmdb_episodes WHERE series_id = ?", (series_id,))
    for episode in episodes:
        conn.execute(
            """
            INSERT INTO tmdb_episodes (
                series_id, tmdb_episode_id, season_number, episode_number, title,
                overview, air_date, runtime, still_path, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                series_id,
                episode.get("tmdb_episode_id"),
                episode["season_number"],
                episode["episode_number"],
                episode["title"],
                episode.get("overview") or "",
                episode.get("air_date"),
                episode.get("runtime"),
                episode.get("still_path"),
                now,
            ),
        )

    conn.execute("DELETE FROM tmdb_series_tags WHERE series_id = ?", (series_id,))
    for tag in classification["tags"]:
        conn.execute("INSERT OR IGNORE INTO tags (name) VALUES (?)", (tag,))
        tag_id = conn.execute("SELECT id FROM tags WHERE name = ?", (tag,)).fetchone()[0]
        conn.execute(
            "INSERT OR IGNORE INTO tmdb_series_tags (series_id, tag_id) VALUES (?, ?)",
            (series_id, tag_id),
        )
    conn.commit()


def connect_db(database_url: str) -> sqlite3.Connection:
    path = sqlite_path(database_url)
    Path(path).parent.mkdir(parents=True, exist_ok=True)
    conn = sqlite3.connect(path)
    conn.execute("PRAGMA foreign_keys = ON")
    return conn


def sqlite_path(database_url: str) -> str:
    if database_url.startswith("sqlite://"):
        return database_url.removeprefix("sqlite://")
    return database_url


def ensure_schema(conn: sqlite3.Connection) -> None:
    statements = [
        """
        CREATE TABLE IF NOT EXISTS tmdb_series (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tmdb_id INTEGER NOT NULL UNIQUE,
            name TEXT NOT NULL,
            original_name TEXT NOT NULL DEFAULT '',
            overview TEXT NOT NULL DEFAULT '',
            first_air_date TEXT,
            poster_path TEXT,
            platform TEXT NOT NULL DEFAULT '',
            age_range TEXT NOT NULL DEFAULT '',
            episode_context_count INTEGER NOT NULL DEFAULT 0,
            llm_reason TEXT NOT NULL DEFAULT '',
            confidence REAL,
            source_url TEXT NOT NULL DEFAULT '',
            updated_at TEXT NOT NULL
        )
        """,
        """
        CREATE TABLE IF NOT EXISTS tmdb_episodes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            series_id INTEGER NOT NULL,
            tmdb_episode_id INTEGER,
            season_number INTEGER NOT NULL,
            episode_number INTEGER NOT NULL,
            title TEXT NOT NULL,
            overview TEXT NOT NULL DEFAULT '',
            air_date TEXT,
            runtime INTEGER,
            still_path TEXT,
            updated_at TEXT NOT NULL,
            UNIQUE(series_id, season_number, episode_number),
            FOREIGN KEY (series_id) REFERENCES tmdb_series(id) ON DELETE CASCADE
        )
        """,
        """
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )
        """,
        """
        CREATE TABLE IF NOT EXISTS tmdb_series_tags (
            series_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (series_id, tag_id),
            FOREIGN KEY (series_id) REFERENCES tmdb_series(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )
        """,
    ]
    for statement in statements:
        conn.execute(statement)
    conn.commit()


def http_json(
    method: str,
    url: str,
    headers: dict[str, str] | None = None,
    body: dict[str, Any] | None = None,
) -> dict[str, Any]:
    data = None
    request_headers = dict(headers or {})
    if body is not None:
        data = json.dumps(body, ensure_ascii=False).encode("utf-8")
        request_headers["Content-Type"] = "application/json"

    req = request.Request(url, data=data, headers=request_headers, method=method)
    for attempt in range(3):
        try:
            with request.urlopen(req, timeout=120) as response:
                return json.loads(response.read().decode("utf-8"))
        except error.HTTPError:
            raise
        except error.URLError:
            if attempt == 2:
                raise
            time.sleep(1 + attempt)
    raise RuntimeError("unreachable")


def read_http_error(exc: error.HTTPError) -> str:
    try:
        body = exc.read().decode("utf-8", errors="replace").strip()
    except Exception:
        body = ""
    return body or exc.reason


def truncate(value: str, limit: int) -> str:
    value = re.sub(r"\s+", " ", value.strip())
    if len(value) <= limit:
        return value
    return value[: limit - 1].rstrip() + "..."


def iso_now() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except KeyboardInterrupt:
        raise SystemExit(130)
