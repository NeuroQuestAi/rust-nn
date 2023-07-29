<img src="https://raw.githubusercontent.com/NeuroQuestAi/five-factor-e/main/doc/neuro-quest.png" align="right" width="80" height="70"/>

# A Simple Neural Network 🧠

[![Powered by NeuroQuestAI](https://img.shields.io/badge/powered%20by-NeuroQuestAI-orange.svg?style=flat&colorA=E1523D&colorB=007D8A)](
https://neuroquest.ai)
![rustc](https://img.shields.io/static/v1.svg?label=rustc&message=1.70%20&color=orange)

This code template is a simple neural network implementation using the [tch](https://github.com/LaurentMazare/tch-rs) ([Torch](https://pytorch.org/)) 
library to perform digit classification on the [MNIST](http://yann.lecun.com/exdb/mnist/) dataset. The [MNIST](http://yann.lecun.com/exdb/mnist/) dataset 
consists of 28x28 grayscale images of handwritten digits (0 to 9). The goal is to train a neural network to correctly classify these digits.

You can use this template to build your Neural Network projects with [Rust](https://www.rust-lang.org/) ➕ [Torch](https://pytorch.org/).

### Project ☁️

Download the project from Git:

```shell
$ git clone https://github.com/NeuroQuestAi/rust-nn.git && cd rust-nn
```

### Requirements 🛠️

The tests were done using a Linux 🐧 machine.

- Need to have installed [build-essential](https://packages.debian.org/pt-br/sid/build-essential);
- You need to have [Rust](https://www.rust-lang.org/) installed.

### Torch 💻

The torch version used was version **v2.0.0**. You can download it directly from the website [https://pytorch.org/get-started/locally/](https://pytorch.org/get-started/locally/), 
or follow the procedures below via the command line. Package used: (libtorch-cxx11-abi-shared-with-deps-2.0.0+cu118.zip).

In the root folder of the project, create a **libs** directory in the root of the project:

```shell
$ mkdir libs
```

Now download the Torch lib and unzip it:

```shell
$ curl -L \
  "https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.0.0%2Bcu118.zip"\
  > libs/libtorch-cxx11-abi-shared-with-deps-2.0.0+cu118.zip
```

The package is about 2.3 GB. Now unzip the file:

```shell
$ unzip libs/libtorch-cxx11-abi-shared-with-deps-2.0.0+cu118.zip -d libs/
```

Now it is necessary to export the environment variables:

```shell
$ export LIBTORCH=`pwd`/libs/libtorch
$ export LD_LIBRARY_PATH=`pwd`/libs/libtorch/lib:${LD_LIBRARY_PATH}
```

### Datasets 💻

Create the data directory:

```shell
$ mkdir data
```

Now let's download the [MNIST](http://yann.lecun.com/exdb/mnist/) datasets:

```shell
$ curl -L "http://yann.lecun.com/exdb/mnist/train-images-idx3-ubyte.gz" > data/train-images-idx3-ubyte.gz
$ curl -L "http://yann.lecun.com/exdb/mnist/train-labels-idx1-ubyte.gz" > data/train-labels-idx1-ubyte.gz
$ curl -L "http://yann.lecun.com/exdb/mnist/t10k-images-idx3-ubyte.gz" > data/t10k-images-idx3-ubyte.gz
$ curl -L "http://yann.lecun.com/exdb/mnist/t10k-labels-idx1-ubyte.gz" > data/t10k-labels-idx1-ubyte.gz
```

Unzip all files:

```shell
$ gunzip data/*.gz
```

### Build and Running 🚀

With rust already installed on your machine, just build:

```shell
$ cargo build
```

And run:

```shell
$ cargo run
```

### Authors 👨‍💻

  * [Ederson Corbari](mailto:e@NeuralQuest.ai) 👽
