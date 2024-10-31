# RewardsMaximizer

| Command           | Builds                   | Date/time      | 
| :---------------- | :------:                 | :------:       |
| cargo run         |   :heavy_check_mark:     | 30OCT24 @ 2015 | 

## API section
### Running backend
1. cargo run
2. Visit [`localhost:3000`](http://localhost:4000)


### Diesel
1. cargo install diesel_cli
#### To run migration: diesel migration run

### Generate database diagram
1.  pip install eralchemy
2.  pip install graphviz
3.  brew install graphviz (if on MacOS) || sudo apt-get install graphviz (Ubuntu)
4.  eralchemy -i sqlite:///database_NAME.db -o output_diagram_NAME.pdf


### Resources:
1. https://doc.rust-lang.org/std/
2. https://doc.rust-lang.org/reference/
3. https://tourofrust.com/TOC_en.html 
4. https://diesel.rs/guides/getting-started