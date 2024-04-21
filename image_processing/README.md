## dependencies

``` 
sudo pacman -S opencv
sudo pacman -S opencv-contrib
sudo pacman -S hdf5 arpack superlu
sudo pacman -S blas lapack

```

## build
``` 
cd build
rm -rf *        
cmake ..
make
```