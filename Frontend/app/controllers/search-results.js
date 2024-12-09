import Controller from '@ember/controller';

export default class Results extends Controller {
  // Get session storage username
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username');
    this.value = sessionStorage.getItem('value');
    this.value = this.capitalize(this.value);
    console.log(this.value);
    console.log(this.username);
    sessionStorage.setItem('username', this.username);
    this.getRewards(this.value);
  }

  // Capitalize first letter in value
  capitalize(string) {
    return string.charAt(0).toUpperCase() + string.slice(1);
  }

  async getRewards() {
    try {
      const response = await fetch('http://localhost:8080/viewRewards');
      if (response.ok) {
        const rewardsData = await response.json();
        if (this.value === 'Food'){
          const filter = rewardsData.filter(reward => reward.reward_id === 1 || reward.reward_id === 37 || reward.reward_id === 28);
          this.set('model', filter);
        }
        else if (this.value === "Travel"){
          const filter = rewardsData.filter(reward => reward.reward_id === 59 || reward.reward_id === 40 || reward.reward_id === 35);
          this.set('model', filter);
        }
        else {
          const filter = rewardsData.filter(reward => reward.reward_id === 54 || reward.reward_id === 26 || reward.reward_id === 37);
          this.set('model', filter);
        }
        // this.set('model', rewardsData);
      } else {
        console.error('Failed to fetch rewards.');
      }
    } catch (error) {
      console.error('Network error:', error);
    }
  }
}
