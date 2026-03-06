#!/bin/bash

cat "third_order_system.csv" | while IFS= read -r line; do
    echo "$line"
    sleep 0.001
done
