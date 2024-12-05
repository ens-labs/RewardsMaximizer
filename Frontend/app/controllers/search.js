import Controller from '@ember/controller';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Search extends Controller {
  @service router;

  // Future functionality for product search
  @action
  results(event) {
    event.preventDefault();
    this.router.transitionTo('search_results');
  }
}
