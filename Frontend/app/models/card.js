import Model, { attr } from '@ember-data/model';

export default class CardModel extends Model {
  @attr('string') cardId;
  @attr('string') cardName;       // The card's name
  @attr('string') rtype;          // Card type (e.g., "Credit")
  @attr('string') icon;           // The icon for the card
  @attr('string') color;          // The color associated with the card
  @attr('string') benefits;       // Benefits of the card
  @attr('string') category;       // Category of the card (e.g., "Cashback", "Travel", etc.)
  @attr('number') rating;         // Rating of the card (e.g., 0 - 5)
  @attr('string') created;        // Date when the card was created
  @attr('string') updated;        // Last updated date
}
