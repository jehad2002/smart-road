#!/bin/bash

while true; do
    git add .

    git commit -m "Update every minute"

    git push
    sleep 60  
done