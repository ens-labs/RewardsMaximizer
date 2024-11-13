import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class CardDetailsController extends Controller {

  // Set the active class for clicked star
  @action
  setActiveStar(event) {
    const selectedStar = event.target.dataset.star;

    // Loop over all the stars and remove "active" class
    for (let i = 1; i <= 5; i++) {
      const star = document.getElementById(`star${i}`);
      star.classList.remove('active');
    }

    // Add the "active" class to the selected star and all stars before it
    for (let i = 1; i <= selectedStar; i++) {
      const star = document.getElementById(`star${i}`);
      star.classList.add('active');
    }
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
