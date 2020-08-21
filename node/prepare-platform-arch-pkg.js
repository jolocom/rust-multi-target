const path = require('path')
const fs = require('fs-extra')

const baseDir = path.resolve(__dirname)
process.chdir(baseDir)

const tmplPkgJsonPath = path.join(baseDir, 'package.json')
const outDir = path.resolve(__dirname, 'platform-arch-pkg')
const outPkgJsonPath = path.join(outDir, 'package.json')

fs.removeSync(outDir)
fs.mkdirpSync(outDir)
const pkgJson = JSON.parse(fs.readFileSync(tmplPkgJsonPath).toString())

const platformArch = `${process.platform}-${process.arch}`
pkgJson.cpu = [process.arch]
pkgJson.os = [process.platform]
pkgJson.name += `-${platformArch}`
pkgJson.description += ` [${platformArch}]`

delete pkgJson.optionalDependencies
delete pkgJson.scripts

fs.writeFileSync(outPkgJsonPath, JSON.stringify(pkgJson, null, 2))
pkgJson.files.forEach(p => {
  const outPath = path.join(outDir, p)
  fs.copySync(p, outPath)
})


console.log(
  '\n\n\n' +
  'Generated ' + outDir +
  '\n\n\n'
)
