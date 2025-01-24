// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "./Base64.sol";

//
// @title SpaghettETHPassport
// Created by: YOMI <dev@yomi.digital>
//

contract PassportResolver is Ownable {
    string public default_name = "SpaghettETH Passport";
    string public default_description = "A passport for the SpaghettETH world";
    string public default_image = "https://ipfs:io/ipfs/QmZ";
    mapping(address => string) public _names;
    mapping(address => string) public _descriptions;
    mapping(address => string) public _images;
    mapping(address => bool) public _proxies;
    mapping(string => address) public _owners;
    event PassportSet(address indexed user, string name, string description, string image);

    // Render token URI
    function tokenURI(
        address owner,
        string[] memory checkpoints,
        string[] memory events
    ) public view returns (string memory) {
        // Render as default values
        string memory rendered_name = default_name;
        string memory rendered_description = default_description;
        string memory rendered_image = default_image;
        // Check if custom name, description, or image is set
        if (bytes(_names[owner]).length > 0) {
            rendered_name = _names[owner];
        }
        if (bytes(_descriptions[owner]).length > 0) {
            rendered_description = _descriptions[owner];
        }
        if (bytes(_images[owner]).length > 0) {
            rendered_image = _images[owner];
        }
        string memory header = string(
            abi.encodePacked(
                '{"name": "',
                rendered_name,
                '", "description": "',
                rendered_description,
                '", "image": "',
                rendered_image,
                '", "attributes": ['
            )
        );

        // Add checkpoints for owner
        string memory attributes = string(abi.encodePacked(""));
        for (uint i = 0; i < checkpoints.length; i++) {
            string memory checkpoint = checkpoints[i];
            attributes = string(
                abi.encodePacked(
                    attributes,
                    '{"trait_type": "',
                    checkpoint,
                    '", "value": "',
                    events[i],
                    '"}'
                )
            );
            if (i < checkpoints.length - 1) {
                attributes = string(abi.encodePacked(attributes, ","));
            }
        }

        string memory data = string(abi.encodePacked(header, attributes, "]}"));
        string memory json = Base64.encode(bytes(data));
        string memory output = string(
            abi.encodePacked("data:application/json;base64,", json)
        );
        return output;
    }

    // Set profile
    function setProfile(
        string memory _name,
        string memory _description,
        string memory _image,
        address user
    ) external {
        if (user != msg.sender) {
            require(_proxies[msg.sender], "SpaghettETH: Not authorized");
        }
        require(
            _owners[_name] == address(0) || _owners[_name] == user,
            "SpaghettETH: Name already taken from another user"
        );
        _names[user] = _name;
        _descriptions[user] = _description;
        _images[user] = _image;
        emit PassportSet(user, _name, _description, _image);
    }

    function setProxyAddress(address proxy, bool state) external onlyOwner {
        _proxies[proxy] = state;
    }
}
