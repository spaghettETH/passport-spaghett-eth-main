// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

//
// @title SpaghettETHPassport
// Created by: YOMI <dev@yomi.digital>
//

interface IRESOLVER {
    function tokenURI(
        address owner,
        string[] memory checkpoints,
        string[] memory events
    ) external view returns (string memory);
}

contract SpaghettETHPassport is ERC721, Ownable {
    struct checkpoint {
        string eventName;
        string description;
        uint256 timestampStart;
        uint256 timestampEnd;
        string checkpointSlug;
        string checkpointImage;
    }

    uint256 private _tokenIdCounter;
    address private _resolverAddress;
    uint256 public _checkpointsCounter;
    string public _baseTokenURI;
    mapping(string => uint256) public _checkpointIds;
    mapping(uint256 => checkpoint) public _checkpoints;
    mapping(address => mapping(uint256 => bool)) public _checkins;
    mapping(address => uint256) public _checkinsCounter;
    mapping(address => bool) public _proxies;
    mapping(address => uint256) public _minted;
    mapping(uint256 => bool) public _burned;
    // Profiles
    mapping(address => string) public _names;
    mapping(address => string) public _descriptions;
    mapping(address => string) public _images;

    event Minted(uint256 indexed tokenId, address indexed owner);
    event Burned(uint256 indexed tokenId);
    event CheckpointSigned(
        uint256 checkpoint,
        address indexed owner,
        string indexed description
    );
    event CheckpointAdded(
        uint256 checkpointId,
        string name,
        string indexed description
    );

    constructor(
        string memory _name,
        string memory _ticker,
        address resolverAddress
    ) ERC721(_name, _ticker) {
        _resolverAddress = resolverAddress;
    }

    function totalSupply() public view returns (uint256) {
        return _tokenIdCounter;
    }

    function mintPassport(address to) external {
        require(_minted[to] == 0, "SpaghettETHPassport: Already minted");
        _tokenIdCounter += 1;
        uint256 tokenId = _tokenIdCounter;
        _safeMint(to, tokenId);
        _minted[to] = tokenId;
        emit Minted(tokenId, to);
    }

    function tokenURI(uint256 id) public view override returns (string memory) {
        if (
            keccak256(abi.encodePacked(_baseTokenURI)) !=
            keccak256(abi.encodePacked(""))
        ) {
            return
                string(abi.encodePacked(_baseTokenURI, Strings.toString(id)));
        }
        address owner = ownerOf(id);
        string[] memory checkpoints = new string[](_checkinsCounter[owner]);
        string[] memory descriptions = new string[](_checkinsCounter[owner]);
        for (uint256 i = 0; i < _checkpointsCounter; i++) {
            if (_checkins[owner][i]) {
                checkpoints[i] = _checkpoints[i].eventName;
                descriptions[i] = _checkpoints[i].description;
            }
        }
        string memory output = IRESOLVER(_resolverAddress).tokenURI(
            owner,
            checkpoints,
            descriptions
        );
        return output;
    }

    function burn(uint256 tokenId) public {
        require(
            _proxies[msg.sender] || ownerOf(tokenId) == msg.sender,
            "SpaghettETHPassport: Only proxy or owner can burn"
        );
        require(!_burned[tokenId], "SpaghettETHPassport: Already burned");
        _burned[tokenId] = true;
        _burn(tokenId);
        _minted[msg.sender] = 0;
        emit Burned(tokenId);
    }

    function setProxyAddress(address proxy, bool state) external onlyOwner {
        _proxies[proxy] = state;
    }

    function setResolverAddress(address resolver) external onlyOwner {
        _resolverAddress = resolver;
    }

    function setBaseUri(string memory baseUri) external onlyOwner {
        _baseTokenURI = baseUri;
    }

    function addCheckpoint(
        string memory eventName,
        string memory description,
        uint256 timestampStart,
        uint256 timestampEnd,
        string memory checkpointSlug,
        string memory checkpointImage
    ) external returns (uint256) {
        require(
            _proxies[msg.sender],
            "SpaghettETHPassport: Only proxy can add checkpoints"
        );
        require(
            keccak256(abi.encodePacked(eventName)) !=
                keccak256(abi.encodePacked("")),
            "SpaghettETHPassport: Checkpoint name not valid"
        );
        require(
            _checkpointIds[checkpointSlug] == 0,
            "SpaghettETHPassport: Checkpoint already exists"
        );
        _checkpoints[_checkpointsCounter].eventName = eventName;
        _checkpoints[_checkpointsCounter].description = description;
        _checkpoints[_checkpointsCounter].timestampStart = timestampStart;
        _checkpoints[_checkpointsCounter].timestampEnd = timestampEnd;
        _checkpoints[_checkpointsCounter].checkpointSlug = checkpointSlug;
        _checkpoints[_checkpointsCounter].checkpointImage = checkpointImage;
        emit CheckpointAdded(_checkpointsCounter, eventName, description);
        _checkpointIds[checkpointSlug] = _checkpointsCounter;
        _checkpointsCounter++;
        return _checkpointsCounter - 1;
    }

    function editCheckpoint(
        string memory checkpointSlug,
        string memory eventName,
        string memory description,
        uint256 timestampStart,
        uint256 timestampEnd,
        string memory checkpointImage
    ) external {
        require(
            _proxies[msg.sender],
            "SpaghettETHPassport: Only proxy can add checkpoints"
        );
        uint256 checkpointId = _checkpointIds[checkpointSlug];
        require(checkpointId != 0, "SpaghettETHPassport: Checkpoint not found");
        _checkpoints[checkpointId].eventName = eventName;
        _checkpoints[checkpointId].description = description;
        _checkpoints[checkpointId].timestampStart = timestampStart;
        _checkpoints[checkpointId].timestampEnd = timestampEnd;
        _checkpoints[checkpointId].checkpointImage = checkpointImage;
    }

    function signCheckpoint(uint256 checkpointId, address owner) external {
        require(
            _proxies[msg.sender],
            "SpaghettETHPassport: Only proxy can add checkpoints"
        );
        require(
            _minted[owner] != 0,
            "SpaghettETHPassport: Passport not minted"
        );
        require(
            !_checkins[owner][checkpointId],
            "SpaghettETHPassport: Check-in already added"
        );
        require(
            block.timestamp >= _checkpoints[checkpointId].timestampStart &&
                block.timestamp <= _checkpoints[checkpointId].timestampEnd,
            "SpaghettETHPassport: Checkpoint not active"
        );
        _checkins[owner][checkpointId] = true;
        _checkinsCounter[owner] += 1;
        emit CheckpointSigned(
            checkpointId,
            owner,
            _checkpoints[checkpointId].description
        );
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId
    ) internal override {
        require(
            from == address(0) || _burned[tokenId],
            "SpaghettETHPassport: Can't transfer passport"
        );
        super._beforeTokenTransfer(from, to, tokenId);
    }
}
