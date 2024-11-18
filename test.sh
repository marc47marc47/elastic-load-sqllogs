./elastic-sqllogs-drop.sh

./elastic-create-indices.sh
./elastic-list-indices.sh
cargo build --release
time ( target/release/elastic-load-sqllogs >run.log )
time ( ./elastic-sqllogs-agg.sh >agg.log )

