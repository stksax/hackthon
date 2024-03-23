pragma circom 2.1.5;
include "../../node_modules/circomlib/circuits/eddsaposeidon.circom";
include "../../node_modules/circomlib/circuits/poseidon.circom";

template verify_vip(){
    signal input Ax;
    signal input Ay;
    signal input S;
    signal input R8x;
    signal input R8y;
    signal input Message;
    signal input agree;

    signal M <== Poseidon(2)(Message, agree);
    component EdDSAPoseidonVerifier();
  
    EdDSAPoseidonVerifier.enabled <== 1;
    EdDSAPoseidonVerifier.Ax <== Ax;
    EdDSAPoseidonVerifier.Ay <== Ay;
    EdDSAPoseidonVerifier.S <== S;
    EdDSAPoseidonVerifier.R8x <== R8x;
    EdDSAPoseidonVerifier.R8y <== R8y;
    EdDSAPoseidonVerifier.M <== M;

    signal output out[2] <== [Ax, Ay]; 
}