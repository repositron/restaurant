curl -d '{"item_name":"pasta", "cooktimeseconds": 20}' -H 'Content-Type: application/json' http://localhost:8080/menu | jq .
curl -v http://localhost:8080/menu


docker-compose up -d postgres

docker-compose down


DATABASE_URL=postgres://res:res@localhost:5434/restaurant

sudo lsof -i -P -n | grep LISTEN
diesel setup
diesel migration generate create_order_table


diesel migration run
diesel migration redo


1. Create a session
2. Create a orders using session_id & table_id
3. Kitchen driver updates order status
4. 




