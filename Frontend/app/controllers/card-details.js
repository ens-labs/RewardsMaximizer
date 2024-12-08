import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class CardDetailsController extends Controller {
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username') || 'Guest';
    console.log(this.username);
    sessionStorage.setItem('username', this.username);
  }

  rating = 0; // Store the rating state

  @action
  setRating(event) {
    const selectedRating = parseInt(event.target.dataset.star, 10);
    this.set('rating', selectedRating);
    console.log(`User rated ${selectedRating} stars.`);
  }
}
