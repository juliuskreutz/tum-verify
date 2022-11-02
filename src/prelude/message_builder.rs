use crate::serenity;

pub trait MessageBuilder {
    fn content<D: ToString>(&mut self, content: D) -> &mut Self;
    fn embed<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut serenity::CreateEmbed) -> &mut serenity::CreateEmbed;
    fn components<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut serenity::CreateComponents) -> &mut serenity::CreateComponents;
}

impl<'a> MessageBuilder for serenity::EditMessage<'a> {
    fn content<D: ToString>(&mut self, content: D) -> &mut Self {
        self.content(content)
    }

    fn embed<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut serenity::CreateEmbed) -> &mut serenity::CreateEmbed,
    {
        self.embed(f)
    }

    fn components<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut serenity::CreateComponents) -> &mut serenity::CreateComponents,
    {
        self.components(f)
    }
}

impl<'a> MessageBuilder for serenity::CreateMessage<'a> {
    fn content<D: ToString>(&mut self, content: D) -> &mut Self {
        self.content(content)
    }

    fn embed<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut serenity::CreateEmbed) -> &mut serenity::CreateEmbed,
    {
        self.embed(f)
    }

    fn components<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut serenity::CreateComponents) -> &mut serenity::CreateComponents,
    {
        self.components(f)
    }
}

impl<'a> MessageBuilder for serenity::CreateInteractionResponseData<'a> {
    fn content<D: ToString>(&mut self, content: D) -> &mut Self {
        self.content(content)
    }

    fn embed<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut serenity::CreateEmbed) -> &mut serenity::CreateEmbed,
    {
        self.embed(f)
    }

    fn components<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut serenity::CreateComponents) -> &mut serenity::CreateComponents,
    {
        self.components(f)
    }
}
