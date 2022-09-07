import fs from 'fs'

import toml from '@ltd/j-toml'

// Deserialize
const workspaceCargo = toml.parse(fs.readFileSync('Cargo.toml'))
const openapiCargo = toml.parse(fs.readFileSync('crates/api/Cargo.toml'))

// Me too :)
const me = workspaceCargo['workspace']['package']['authors'][0]

if (!openapiCargo['package']['authors'].includes(me)) {
    openapiCargo['package']['authors'].push(me)
}

// Inherit the workspace package table.
openapiCargo['package']['license'] = { 'workspace': true };
openapiCargo['package']['description'] = { 'workspace': true };
openapiCargo['package']['repository'] = { 'workspace': true };

// Serialize
fs.writeFileSync(
    'crates/api/Cargo.toml',
    toml.stringify(openapiCargo, {
        newline: '\n',
        newlineAround: 'section',
    }).replaceAll('\'', '\"')
)
