### Install rust kernel
``` 
cargo install --locked evcxr_jupyter
``` 

### Register kernel
``` 
evcxr_jupyter --install
``` 

### Install Rust standard library (if rust analyzer is installed, it is probably in place)
```
rustup component add rust-src
```

### Create folder to hold your notebooks and python venv: 
``` 
mkdir jupyter-rust
```

### Create python virtual environement  
``` 
python -m venv venv
``` 

### Activate environment
``` 
. venv/Script/activate
``` 

### Install jupyter
``` 
pip install jupyter
``` 

### Start Jupyter
``` 
jupyter notebook
``` 

### Add crate dependencies in notebook

``` 
:dep polars = { version = "0.43", features = ["lazy", "dtype-date", "dtype-datetime", "parquet"] }

use polars::prelude::*;
``` 

### Create a datafrrame with polars
``` 
let df = df! [
    "name" => &["Alice", "Bob"],
    "age" => &[30i64, 25i64],
    "city" => &["NYC", "LA"],
]?;

println!("{:?}", df);
``` 

