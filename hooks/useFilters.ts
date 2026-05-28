import { useState, useMemo } from 'react';

export function useFilters<T>(data: T[], filterKey: keyof T) {
  // null signifie "Tous" (aucun filtre appliqué)
  const [activeFilter, setActiveFilter] = useState<string | null>(null);

  // Mémorisation pour éviter les recalculs inutiles à chaque rendu
  const filteredData = useMemo(() => {
    if (!activeFilter) return data;
    return data.filter((item) => String(item[filterKey]) === activeFilter);
  }, [data, activeFilter, filterKey]);

  // Extraction dynamique des filtres disponibles (KISS)
  const availableOptions = useMemo(() => {
    const options = new Set(data.map((item) => String(item[filterKey])));
    return Array.from(options);
  }, [data, filterKey]);

  return {
    filteredData,
    activeFilter,
    setActiveFilter,
    availableOptions,
  };
}