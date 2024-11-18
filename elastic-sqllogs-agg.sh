curl -X POST "${ELASTIC_URL}/sql_logs/_search?pretty" \
-H "Content-Type: application/json" \
-d '{
  "size": 0,
  "aggs": {
    "by_exec_time": {
      "date_histogram": {
        "field": "exec_time",
        "fixed_interval": "5m",
        "time_zone": "UTC"
      },
      "aggs": {
        "by_client_ip": {
          "terms": {
            "field": "client_ip.keyword",
            "size": 100
          },
          "aggs": {
            "by_sql_type": {
              "terms": {
                "field": "sql_type.keyword",
                "size": 100
              },
              "aggs": {
                "record_count": {
                  "value_count": {
                    "field": "conn_hash"
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}'

