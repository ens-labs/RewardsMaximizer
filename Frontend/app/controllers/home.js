import Controller from '@ember/controller';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Home extends Controller {
  @service router;

  // Navigate to wallet
  @action
  wallet(event) {
    event.preventDefault();
    this.router.transitionTo('wallet');
  }

  // Navigate to search
  @action
  search(event) {
    event.preventDefault();
    this.router.transitionTo('search');
  }

  // Navigate to profile
  // @action
  // profile(event) {
  //   event.preventDefault();
  //   this.router.transitionTo('profile');
  // }

  // Navigate to crowdsourcing
  @action
  crowdsourcing(event) {
    event.preventDefault();
    this.router.transitionTo('crowdsourcing');
  }
}
