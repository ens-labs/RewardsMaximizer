import EmberRouter from '@ember/routing/router';
import config from 'frontend/config/environment';

export default class Router extends EmberRouter {
  location = config.locationType;
  rootURL = config.rootURL;
}

Router.map(function () {
  this.route('login');
  this.route('home');
  this.route('wallet');
  this.route('search');
  this.route('search_results');
  this.route('crowdsourcing');
});
