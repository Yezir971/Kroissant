//! Template HTML pour l'email de vérification.

pub fn render(token_url: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <title>Vérifiez votre adresse e-mail — Kroissant</title>
</head>
<body style="margin: 0; padding: 0; background-color: #FAF7F2; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;">
    <table border="0" cellpadding="0" cellspacing="0" width="100%" bgcolor="#FAF7F2">
        <tr>
            <td align="center" style="padding: 20px 0;">
                <!--[if (gte mso 9)|(IE)]>
                <table align="center" border="0" cellspacing="0" cellpadding="0" width="600">
                <tr>
                <td align="center" valign="top" width="600">
                <![endif]-->
                <table border="0" cellpadding="0" cellspacing="0" width="100%" style="max-width: 600px; background-color: #FFFFFF; border-radius: 12px; overflow: hidden;">
                    <tr>
                        <td style="padding: 40px;">
                            <table border="0" cellpadding="0" cellspacing="0" width="100%">
                                <tr>
                                    <td align="center" style="padding-bottom: 32px;">
                                        <div style="color: #A0522D; font-size: 28px; font-weight: bold; text-decoration: none;">KROISSANT</div>
                                    </td>
                                </tr>
                                <tr>
                                    <td style="padding-bottom: 16px;">
                                        <div style="font-size: 20px; font-weight: 600; color: #2D2D2D;">Bonjour ! 👋</div>
                                    </td>
                                </tr>
                                <tr>
                                    <td style="padding-bottom: 32px;">
                                        <p style="font-size: 16px; line-height: 1.6; color: #555555; margin: 0;">
                                            Merci de vous être inscrit sur Kroissant, la plateforme de contenus pour enfants sélectionnés selon des critères scientifiques.
                                        </p>
                                    </td>
                                </tr>
                                <tr>
                                    <td style="padding-bottom: 32px;">
                                        <p style="font-size: 16px; line-height: 1.6; color: #555555; margin: 0;">
                                            Cliquez sur le bouton ci-dessous pour vérifier votre adresse e-mail.
                                        </p>
                                    </td>
                                </tr>
                                <tr>
                                    <td align="center" style="padding-bottom: 32px;">
                                        <a href="{}" target="_blank" style="background-color: #A0522D; color: #FFFFFF; display: inline-block; padding: 14px 28px; text-decoration: none; border-radius: 8px; font-size: 16px; font-weight: 600; width: 280px; text-align: center;">
                                            Vérifier mon adresse e-mail
                                        </a>
                                    </td>
                                </tr>
                                <tr>
                                    <td style="border-top: 1px solid #EEEEEE; padding-top: 32px;">
                                        <p style="font-size: 13px; color: #888888; margin: 0 0 8px 0;">
                                            ⚠ Ce lien expire dans 24 heures.
                                        </p>
                                        <p style="font-size: 13px; color: #888888; margin: 0;">
                                            Si vous n'êtes pas à l'origine de cette demande, ignorez cet e-mail.
                                        </p>
                                    </td>
                                </tr>
                            </table>
                        </td>
                    </tr>
                    <tr>
                        <td align="center" bgcolor="#F5F0E8" style="padding: 20px; color: #AAAAAA; font-size: 12px;">
                            <p style="margin: 0 0 4px 0;">&copy; 2025 Kroissant</p>
                            <p style="margin: 0;">Tous droits réservés</p>
                        </td>
                    </tr>
                </table>
                <!--[if (gte mso 9)|(IE)]>
                </td>
                </tr>
                </table>
                <![endif]-->
            </td>
        </tr>
    </table>
</body>
</html>"##,
        token_url
    )
}
