
Download torch:

Download libtorch from https://pytorch.org/get-started/locally/. This package requires v2.0.0: if this version is no longer available on the "get started" page, the file should be accessible by modifying the target link, for example https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.0.0%2Bcu118.zip

mkdir libs

curl -L "https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.0.0%2Bcu118.zip" > libs/libtorch-cxx11-abi-shared-with-deps-2.0.0+cu118.zip

unzip libs/libtorch-cxx11-abi-shared-with-deps-2.0.0+cu118.zip -d libs/

export LIBTORCH=`pwd`/libs/libtorch
export LD_LIBRARY_PATH=`pwd`/libs/libtorch/lib:${LD_LIBRARY_PATH}

=======================

http://yann.lecun.com/exdb/mnist/ 

Download datasets:

mkdir data

curl -L "http://yann.lecun.com/exdb/mnist/train-images-idx3-ubyte.gz" > data/train-images-idx3-ubyte.gz
curl -L "http://yann.lecun.com/exdb/mnist/train-labels-idx1-ubyte.gz" > data/train-labels-idx1-ubyte.gz

curl -L "http://yann.lecun.com/exdb/mnist/t10k-images-idx3-ubyte.gz" > data/t10k-images-idx3-ubyte.gz
curl -L "http://yann.lecun.com/exdb/mnist/t10k-labels-idx1-ubyte.gz" > data/t10k-labels-idx1-ubyte.gz

gunzip data/*.gz

======================

cargo build
cargo run


