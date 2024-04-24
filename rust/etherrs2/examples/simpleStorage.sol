pragma solidity ^0.8.23;

contract SimpleStorage {
    uint storageData;
    string public name;

    constructor(string memory _name) {
        name = _name;
    }
    function get() public returns(uint) {
        return storageData;
    }

    function set(uint n) public {
        storageData = n;
    }

    function increment(uint n) public {
        storageData = storageData + n;
    }

    function decrement(uint n) public {
        storageData = storageData - n;
    }

}
