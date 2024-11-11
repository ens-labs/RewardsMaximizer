import Controller from '@ember/controller';
import {action} from '@ember/object';
import {service} from '@ember/service';

export default class Crowdsourcing extends Controller {
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
    search(event) {
      event.preventDefault();
      this.router.transitionTo('search');
    }

    // Navigate to crowdsourcing
    @action
    crowdsourcing(event) {
      event.preventDefault();
      this.router.transitionTo('crowdsourcing');
    }
}