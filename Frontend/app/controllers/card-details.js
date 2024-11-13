import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class CardDetailsController extends Controller {
  rating = 0; // Store the rating state

  // Set the rating when a user clicks a star
  @action
  setRating(event) {
    const selectedRating = parseInt(event.target.dataset.star, 10); // Get the rating from the data-star attribute
    this.set('rating', selectedRating); // Update the rating state
  }

  // Navigate to other pages
  @action
  home(event) {
    event.preventDefault();
    this.router.transitionTo('home');
  }

  @action
  wallet(event) {
    event.preventDefault();
    this.router.transitionTo('wallet');
  }

  @action
  profile(event) {
    event.preventDefault();
    this.router.transitionTo('profile');
  }

  @action
  crowdsourcing(event) {
    event.preventDefault();
    this.router.transitionTo('crowdsourcing');
  }

  @action
  results(event) {
    event.preventDefault();
    this.router.transitionTo('search_results');
  }
}
