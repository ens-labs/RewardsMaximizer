'use strict';

const EmberApp = require('ember-cli/lib/broccoli/ember-app');

module.exports = function (defaults) {
  const app = new EmberApp(defaults, {
    // Add options here
  });


  app.import('node_modules/jquery/dist/jquery.min.js');

  return app.toTree();
};
