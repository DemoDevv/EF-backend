# Fonction pour charger le fichier .env
function load_env() {
    if [ -f .env ]; then
        echo "Chargement des variables d'environnement depuis .env"

        # Lire le fichier .env ligne par ligne
        while IFS= read -r line || [ -n "$line" ]; do
            # Ignorer les lignes vides ou commençant par #
            case "$line" in
                \#*|"") continue ;;
            esac

            # Exporter la variable
            export "$line"
        done < .env
    else
        echo "Fichier .env non trouvé"
    fi
}
