try {
  // during development, use the build that's directly here
  module.exports = require('../native')
  console.warn('@jolocom/native-core: using native build from main package!')
} catch {
  // this will work as long as the node version is 2 digits long
  module.exports = require(`@jolocom/native-core-node-${process.version.substr(1, 2)}-${process.platform}-${process.arch}`);
}
