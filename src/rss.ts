export interface RssChannelConfig {
  title: string;
  link: string;
  language: string | null;
}

export interface RssItem {
  title: string;
  link: string;
  author: string;
  pub_date: string;
  guid: { value: string; is_permalink: boolean };
  description: string;
}

export function buildRss(config: RssChannelConfig, items: RssItem[]): string {
  const rssitem = items.map((item): string => {
    return (
      "<item>" +
      `<title>${item.title}</title>` +
      `<link>${item.link}</link>` +
      `<author>${item.author}</author>` +
      `<pubDate>${item.pub_date}</pubDate>` +
      `<guid isPermaLink=${item.guid.is_permalink}>${item.guid.value}</guid>` +
      `<description><![CDATA[${item.description}]]></description></item>`
    );
  });

  return (
    `<rss version="2.0">` +
    "<channel>" +
    `<title>${config.title}</title>` +
    `<link>${config.link}</link>` +
    `<language>${config.language ?? ""}</language>` +
    rssitem.join("") +
    "</channel></rss>"
  );
}
