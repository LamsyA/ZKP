# Pedersen Commitment Scheme

The Pedersen commitment scheme is a cryptographic protocol that allows a user to commit to a chosen value while keeping it hidden, with the ability to reveal the committed value later. The commitment scheme has two main properties: hiding and binding. Hiding ensures that the committed value remains secret, and binding ensures that the committer cannot change the value once committed.

### Formula of Pedersen Commitment Scheme

The Pedersen commitment ( C ) to a value ( m ) with a randomness ( r ) is calculated as:

C = g^m h^r mod p

where:

- g and h are generators of a cyclic group of prime order ( q ) (typically subgroups of Z_p^\* ).
- ( m ) is the message or value being committed.
- ( r ) is a random value (often referred to as a nonce or blinding factor).
- ( p ) is a large prime number such that ( p = kq + 1 ) for some integer ( k ).

### Properties of the Scheme

1. **Hiding**: The commitment ( C ) does not reveal any information about ( m ) because of the random value ( r ). Even if an attacker knows ( g ), ( h ), and ( C ), they cannot determine ( m ) without knowing ( r ).

2. **Binding**: Once ( C ) is published, the committer cannot change ( m ) without being detected. This is because finding another pair ((m', r')) such that ( C = g^{m'} h^{r'} mod p ) would require solving the discrete logarithm problem, which is computationally hard.

### Steps in the Pedersen Commitment Scheme

1. **Setup**: Choose a large prime ( p ) and ( q ) such that ( p = kq + 1 ). Select generators ( g ) and ( h ) of the cyclic group of order ( q ).

2. **Commit**: To commit to a value ( m ), choose a random value ( r ). Compute the commitment ( C ) using the formula:
   C = g^m h^r mod p

3. **Reveal**: To reveal the committed value, publish ( m ) and ( r ).

4. **Verify**: To verify the commitment, check that the commitment ( C ) matches the value:
   C = g^m h^r mod p

### Example Explanation

- **Setup**:

  - Choose ( p ) = a large prime number.
  - Choose ( q ) = a prime divisor of ( p-1 ).
  - Select ( g ) and ( h ) such that both are generators of the subgroup of order ( q ) in ( Zp^\* ).

- **Commit**:

  - Suppose ( m ) is the message you want to commit to (e.g., ( m = 42 )).
  - Choose a random ( r ) (e.g., ( r = 14207 )).
  - Compute ( C = g^m h^r mod p ).

- **Reveal**:

  - Publish ( m ) and ( r ).

- **Verify**:
  - The verifier checks that the provided ( m ) and ( r ) satisfy ( C = g^m h^r mod p ).

This scheme ensures that the commitment is secure and can only be opened to the committed value, preventing cheating and ensuring privacy.
