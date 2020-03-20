const addon = require("../native/index.node");

const addonPromise = filePath => {
  return new Promise((resolve, reject) => {
    addon.murmur2(filePath, (err, value) => {
      if (value) resolve(value);
      reject(err);
    });
  });
};

module.exports = addonPromise;
