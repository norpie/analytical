#!/usr/bin/env bash

# Ik beschouw dat het doel bereikt is als de databasesoftware met succes gegevens opslaat en ophaalt voor minimaal 1.000 datapunten.
#
# struct Metric {
#    name: String,
#    key: String,
#    timestamp: u64,
#    value: f64,
#}

# get http://localhost:4340/metrics?name=test&key=test
# post http://localhost:4340/metrics { name: "test", key: "test", timestamp: 1234567890, value: 123.456 }

# Test 1: n datapunten
n=1000
for i in $(seq 1 $n); do
    # current timestamp in nanos and random value between 0 and 1000
    metric="{ \"name\": \"test\", \"key\": \"test\", \"timestamp\": $(
        date +%s%N | cut -b1-13
    ), \"value\": $((RANDOM % 1000)) }"
    curl -X POST -H "Content-Type: application/json" -d "$metric" http://localhost:4340/metrics
    echo "POST $i/$n"
done

# Test 2: ophalen van n datapunten
result=$(curl -X GET "http://localhost:4340/metrics?name=test&key=test")

# Use `jq` to verify the result
if [ $(echo $result | jq '. | length') -eq $n ]; then
    echo "Test succeeded"
else
    echo "Test failed"
fi
