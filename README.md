Project Description
knowledge Management system serves as a solution to the reliance of discussion platforms on centralized databases, which are vulnerable
to data manipulation or unilateral deletion. By utilizing smart contract technology, this system ensures that 
every intellectual contribution from students is stored immutably (cannot be arbitrarily altered) and can only 
be managed through predefined contract functions.

The system supports full CRUD (Create, Read, Update, Delete) operations, allowing users to:

Publish new knowledge.
Access the entire community knowledge base.
Update information or revise thread content.
Delete threads that are no longer relevant.

Key Features
1. Thread Publishing (Create)
Each new thread is recorded with complete attributes: Author, Title, Category, and Content. The system automatically generates a unique ID using a PRNG (Pseudo-Random Number Generator) for each post.

2. Transparent Data Access (Read)
All forum posts can be retrieved in a single function call, enabling easy integration with the user interface (Frontend).

3. Content Revision (Update)
A feature to update the title and content of a thread if there is new information, while maintaining the integrity of the original post ID.

4. Storage Management (Delete)
The ability to permanently delete a thread from the contract storage to ensure data storage efficiency on the blockchain.

Technical Details
Contract ID: CCD2PLUOV3DNJCOFT6X6QLVPQKF4PIPZSBK43TP2ZAXZRKV3PYOJMBMA
Network: Stellar Testnet
Language: Rust
SDK: Soroban SDK