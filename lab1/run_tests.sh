cargo build --release

cp target/release/*.exe ./data -f
cp target/release/*.dll ./data -f

cd data
start ./mr_master
start ./mr_worker