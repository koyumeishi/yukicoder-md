require('dotenv').config();

module.exports = {
  basePath: process.env.GITHUB_PAGES ? '/yukicoder-md' : '',
};
