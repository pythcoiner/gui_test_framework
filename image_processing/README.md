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

## Usage
``` 
./image_processing -f /home/pyth/rust/gui_test_framework/image_processing/frame.png -c '{ "colors": [[254, 167, 0], [226, 78, 27], [0, 255, 0], [127, 0, 127], [255, 105, 180], [228, 171, 183] ]}'
```