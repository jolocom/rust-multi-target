/**
 * This script will create a platform specific package
 * such as '@jolocom/native-core-node-linux-x64' using the
 * parent directory's package.json and build output
 */

const path = require('path')
const fs = require('fs-extra')
const svMajor = require('semver/functions/major')

const baseDir = path.resolve(__dirname)
process.chdir(baseDir)
const nativeDir = path.resolve(baseDir, 'native')

const tmplPkgJsonPath = path.join(nativeDir, 'package.tmpl.json')
const outPkgJsonPath = path.join(nativeDir, 'package.json')

const pkgJson = JSON.parse(fs.readFileSync(tmplPkgJsonPath).toString())

const platformArch = `${process.platform}-${process.arch}`
const majorVersion = svMajor(`${process.version}`)

if (majorVersion > 14 || majorVersion < 8) {
  throw new Error(`Invalid Node Version: ${majorVersion}`)
}

pkgJson.cpu = [process.arch]
pkgJson.os = [process.platform]
pkgJson.engines = { node: `^${majorVersion}.0.0` }
pkgJson.name += `${majorVersion}-${platformArch}`
pkgJson.description += ` [${platformArch}]`

console.log(
  '\n\n\n' +
  'Generating platform specific package: ' + pkgJson.name
)

// write out native/package.json
fs.writeFileSync(outPkgJsonPath, JSON.stringify(pkgJson, null, 2))

console.log(
  'Generated at ' + outPkgJsonPath +
  '\n\n\n' +
  'please do: cd native && npm publish' +
  '\n'
)
