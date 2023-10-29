pragma circom 2.1.6;

template Whitelist() {
    signal input addressInDecimal;
    signal input sameAddressButPublic;
    
    assert(addressInDecimal==sameAddressButPublic);
}

component main {public [sameAddressButPublic]} = Whitelist();