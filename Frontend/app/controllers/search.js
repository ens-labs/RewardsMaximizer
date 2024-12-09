import Controller from '@ember/controller';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Search extends Controller {
  @service router;

  // Get session storage username
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username');
    console.log(this.username);
    sessionStorage.setItem('username', this.username);
  }

  // Set the category value for card search
  @action
  setValue(event) {
    this.value = event.target.value;
    console.log(this.value);
  }

  // Future functionality for product search
  @action
  results(event) {
    event.preventDefault();
    this.router.transitionTo('search_results');
    sessionStorage.setItem('value', this.value);
  }
}
