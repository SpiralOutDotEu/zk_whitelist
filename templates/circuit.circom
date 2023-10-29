pragma circom 2.1.6;

template Whitelist() {
    signal input a;
    signal input b;
    
    assert(a==b);
}

component main {public [b]} = Whitelist();