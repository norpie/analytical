# Analytical

> A suite of analytical tools for software projects.

## Overview

- Storeful: A storage engine for the databases.
- Metrical: A metrics database.
- Logical: A log database.
- Traceful: A trace database.
- Insightful: A frontend for the databases.

### Storeful

Based on RocksDB.

### Metrical

```json
{
    "timestamp": 132412341234,
    "name": "metric_name",
    "labels": {
        "key1": "value1",
        "key2": "value2",
    },
    "value": 0.9341
}
```

or line protocol:

`2024-11-04T20:46:17.651349572Z metric_name{key1="value1", key2="value2"} 0.9341`

### Logical

```json
{
    "timestamp": 132412341234,
    "message": "This is a log message.",
    "labels": {
        "key1": "value1",
        "key2": "value2",
    }
}
```

or line protocol:

`2024-11-04T20:46:17.651349572Z {key1="value1", key2="value2"} This is a log message.`

### Traceful

```json
{
    "labels": {
        "key1": "value1",
        "key2": "value2",
    },
    "events": [
        {
            "timestamp": 132412341234,
            "name": "event1",
            "eventType": "Start",
        },
        {
            "timestamp": 132412341234,
            "name": "event2",
            "eventType": "Annotation",
        },
        {
            "timestamp": 132412341234,
            "name": "event1",
            "eventType": "End",
        }
    ]
}
```

or line protocol:

`{key1="value1"} {2024-11-04T21:07:11.131471192Z event1 Start, 2024-11-04T21:07:11.131477873Z event2 Annotation, 2024-11-04T21:07:11.131478054Z event1 End}`


### Insightful
