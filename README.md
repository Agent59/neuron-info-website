# neuron-info-website
An informational website about neurons


### Deploy using docker
1. Build the image:
```sh
docker build -t neuron-info-website ./
```

2. Run using docker run:
```sh
docker run --name neuron-info-website --expose=8080 --network host -d neuron-info-website
```
