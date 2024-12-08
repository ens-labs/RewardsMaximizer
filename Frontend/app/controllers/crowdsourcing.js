import Controller from '@ember/controller';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';

export default class Crowdsourcing extends Controller {
  @tracked rating = 0; // User rating
  @tracked comment = '';  // User comment
  @tracked created = new Date().toISOString(); 
  @tracked updated = new Date().toISOString(); 
  @tracked companyID = 1; // Hard coded company ID
  @tracked user_id = 0; // User ID

  // Get session storage username
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username');
    console.log(this.username);
    sessionStorage.setItem('username', this.username);
  }

  @action
  setComment(event) {
    this.comment = event.target.value;
  }

  @action
  setRating(event) {
    this.rating = event.target.value;
    console.log(this.rating);
  }

  // Submit reward card recommendation
  // @action
  // submitRecommendation = async () => {
    // Get user_id 
    // try{
    //   const response = await fetch('http://localhost:8080/users/${this.username}');
    //   const data = await response.json();
    //   this.user_id = data.user_id;
    // } 
    // catch (error) {
    //   console.error('Error:', error);
    // }

    // Post recommendation
    // try {
    //   const response = await fetch('http://localhost:8080/crowdsourcing', {
    //     method: 'POST',
    //     headers: {'Content-Type': 'application/json'},
    //     body: JSON.stringify({
    //       user_id: this.user_id,
    //       company_id: this.companyID,
    //       rating: this.rating,
    //       comments: this.comment,
    //       created: this.created,
    //       updated: this.updated,
    //     }),
    //   });

    //   if (response.ok) {
  //       this.errorMessage = 'Recommendation submitted!';
  //     }
  //     else{
  //       const error = await response.text();
  //       this.errorMessage = `Error submitting recommendation: ${error}`;
  //       throw new Error("Recommendation submission failed");
  //     }
  //   } 
  //   catch (error) {
  //     console.error('Error:', error);
  //     this.errorMessage = 'Error submitting recommendation';
  //   }
  // }
}

