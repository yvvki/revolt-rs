import fs from 'fs';

import toml from '@ltd/j-toml';

// Deserialize
const workspaceCargo = toml.parse(fs.readFileSync('Cargo.toml'));
const apiCargo = toml.parse(fs.readFileSync('crates/api/Cargo.toml'));

// Me too :)
const me = workspaceCargo['workspace']['package']['authors'][0];

if (!apiCargo['package']['authors'].includes(me)) {
  apiCargo['package']['authors'].push(me);
}

// Inherit the workspace package table.
apiCargo['package']['license'] = { 'workspace': true };
apiCargo['package']['description'] = { 'workspace': true };
apiCargo['package']['repository'] = { 'workspace': true };

// Serialize
fs.writeFileSync(
  'crates/api/Cargo.toml',
  toml.stringify(apiCargo, {
    newline: '\n',
    newlineAround: 'section',
  }).replaceAll('\'', '\"')
);
