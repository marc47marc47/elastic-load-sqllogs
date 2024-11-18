curl -X POST "${ELASTIC_URL}/sql_logs/_search?pretty" \
-H "Content-Type: application/json" \
-d '{
  "size": 0,
  "aggs": {
    "min_exec_time": {
      "min": {
        "field": "exec_time"
      }
    },
    "max_exec_time": {
      "max": {
        "field": "exec_time"
      }
    }
  }
}'
