#!/bin/bash
# Setup script for PostgreSQL database for projects

set -e

# Database configuration
DB_NAME="yoda_chat"
DB_USER="postgres"
DB_PASSWORD="postgres"
DB_HOST="localhost"
DB_PORT="5432"

echo "Setting up PostgreSQL database for Yoda Chat projects..."

# Check if PostgreSQL is running
if ! pg_isready -h $DB_HOST -p $DB_PORT -q; then
    echo "PostgreSQL is not running on $DB_HOST:$DB_PORT"
    echo "Starting PostgreSQL service..."
    
    # Try to start PostgreSQL (Linux/systemd)
    if command -v systemctl &> /dev/null; then
        sudo systemctl start postgresql
    elif command -v service &> /dev/null; then
        sudo service postgresql start
    else
        echo "Could not start PostgreSQL automatically. Please start it manually."
        exit 1
    fi
    
    # Wait for PostgreSQL to start
    sleep 2
fi

# Check if database exists
if psql -h $DB_HOST -p $DB_PORT -U $DB_USER -lqt | cut -d \| -f 1 | grep -qw $DB_NAME; then
    echo "Database '$DB_NAME' already exists."
else
    echo "Creating database '$DB_NAME'..."
    PGPASSWORD=$DB_PASSWORD createdb -h $DB_HOST -p $DB_PORT -U $DB_USER $DB_NAME
    echo "Database '$DB_NAME' created successfully."
fi

# Apply schema
echo "Applying database schema..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f "$(dirname "$0")/schema.sql"

echo "Database setup complete!"
echo ""
echo "Connection details:"
echo "  URL: postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"
echo ""
echo "To test the connection, run:"
echo "  export DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"
echo "  cargo test --lib project -- --ignored"
