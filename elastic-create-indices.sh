curl -X PUT "${ELASTIC_URL}/sql_logs?pretty" -H "Content-Type: application/json" -d '{
  "mappings": {
    "properties": {
      "conn_hash": { "type": "keyword", "doc_values": true },
      "stmt_id": { "type": "integer" },
      "exec_id": { "type": "integer" },
      "exec_time": { "type": "date", "format": "yyyy-MM-dd HH:mm:ss||epoch_millis" },
      "sql_type": { "type": "keyword", "doc_values": true },
      "exe_status": { "type": "keyword", "doc_values": true },
      "db_ip": { "type": "ip" },
      "client_ip": { "type": "ip" },
      "client_host": { "type": "keyword", "doc_values": true },
      "app_name": { "type": "keyword", "doc_values": true },
      "db_user": { "type": "keyword", "doc_values": true },
      "sql_hash": { "type": "keyword", "doc_values": true },
      "from_tbs": { "type": "keyword", "doc_values": true },
      "select_cols": { "type": "text", "analyzer": "keyword" },
      "sql_stmt": { "type": "text", "analyzer": "standard" },
      "stmt_bind_vars": { "type": "text", "analyzer": "standard" }
    }
  }
}'
