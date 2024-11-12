import Controller from '@ember/controller';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Search extends Controller {
  @service router;

  // Navigate to home
  @action
  home(event) {
    event.preventDefault();
    this.router.transitionTo('home');
  }

  // Navigate to wallet
  @action
  wallet(event) {
    event.preventDefault();
    this.router.transitionTo('wallet');
  }

  // Navigate to profile
  @action
  profile(event) {
    event.preventDefault();
    this.router.transitionTo('profile');
  }

  // Navigate to crowdsourcing
  @action
  crowdsourcing(event) {
    event.preventDefault();
    this.router.transitionTo('crowdsourcing');
  }

  // Future functionality for product search
  @action
  results(event) {
    event.preventDefault();
    this.router.transitionTo('search_results');
  }
}
