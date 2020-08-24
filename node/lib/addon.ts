try {
  // during development, use the build that's directly here
  module.exports = require('../native')
  console.warn('@jolocom/native-core: using native build from main package!')
} catch {
  module.exports = require(`@jolocom/native-core-node-${process.platform}-${process.arch}`);
}
