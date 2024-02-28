#!/bin/bash

# Change to the directory of the <init.sh> script
cd "$(dirname "$0")"

# Create a new SQLite database
sqlite3 db.db ".databases"

# Execute the SQL script
sqlite3 db.db < seed_db.sql

#validate
sqlite3 db.db "SELECT * FROM Candidate;"