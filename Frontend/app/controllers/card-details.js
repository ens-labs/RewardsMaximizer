import Controller from '@ember/controller';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class CardDetailsController extends Controller {
  @service router;

  rating = 0; // Store the rating state

  // Assuming cardIcon is coming from the model data
  cardIcon = 'fas fa-credit-card'; // Example: FontAwesome class

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

  // Navigate to search
  @action
  search(event) {
    event.preventDefault();
    this.router.transitionTo('search');
  }
}
