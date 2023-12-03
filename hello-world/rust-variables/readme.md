# rust-variables

### four scalar types variable:
- integers (signed / unsigned)
- float
- char
- bool

### compound type variables:
- array (0 based indexing)
- tuple (holds unlimited elements of diff types unlike array)

## variable mutability
You can't modify the data type. 
For example: 
```
let num = 2;
num = 4; 
```
This will throw an error. 

To change the variable, use the keyword **mut**.
```
let mut num = 2;
num = 4; 
```

## array and slices
- An array is a reference (doesn't points to the value but the address/memory)
- slice is a sub array 
```
let arr = [0,1,2,3];
let slice = &arr[1 .. 3]; // [1, 2] / & = dereferencing
```

**The difference:** In array, we know the length at compile time. In slices, we don't know the lenght. 

