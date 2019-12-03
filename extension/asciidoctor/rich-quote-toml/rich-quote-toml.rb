module Lo48576
  require 'toml-rb'
  require 'asciidoctor/extensions'
  include Asciidoctor

  module Helper
    def to_inline_html parent, source_adoc
      para = create_paragraph parent, nil, {}
      parse_content para, source_adoc
      para.blocks[0].content
    end
  end

  # ```
  # [richquote]
  # ====
  # # Quoted text.
  # # String.
  # text = '''
  #
  # # Display name of the creator of the quoted content (optional).
  # # AsciiDoc string.
  # creator = "@user"
  #
  # Quoted text in AsciiDoc format.
  # '''
  # # Document or event where the quoted content (firstly) appeared.
  # # AsciiDoc string.
  # source = 'Quote source in inline AsciiDoc format.'
  #
  # # URI of the quote source, if available.
  # # Array of strings or string.
  # uri = 'https://example.com/'
  #
  # # Published date of the quoted content (optional).
  # published = '2000-01-01T00:00:00+09:00'
  # # Updated date of the quoted content (optional).
  # updated = '2000-01-01T00:00:00+09:00'
  # # Datetime when the content is referred and quoted.
  # referred = '2000-01-01T00:00:00+09:00'
  #
  # # Note about the quote (optional).
  # # AsciiDoc string.
  # note = '*emphasis* added'
  # ====
  # ```
  class RichQuoteBlock < Extensions::BlockProcessor
    use_dsl
    named :richquote
    on_context [:paragraph, :open, :example, :quote, :listing]
    content_model :compound

    include Helper

    def process parent, reader, attrs
      doc = parent.document
      if !(doc.basebackend? 'html')
        raise "Unsupported backend #{doc.basebackend}"
      end
      separator = ', '

      props = TomlRB.parse reader.read_lines.join("\n")

      figure_content = create_pass_block parent, nil, attrs, content_model: :compound

      text = create_pass_block figure_content, nil, attrs, content_model: :compound
      parse_content text, props['text']
      uri = props['uri']
      blockquote_html = %(<blockquote#{%[ cite="#{uri}"] if uri}>
#{text.convert}
</blockquote>)
      figure_content << (create_pass_block figure_content, blockquote_html, attrs, subs: nil)

      figure_footer_content = []
      if (creator = props['creator'])
        figure_footer_content << creator
      end
      if (source = props['source'])
        figure_footer_content << source
      end
      if (note = props['note'])
        figure_footer_content << note
      end

      figcaption_html = %(<figcaption>#{to_inline_html parent, figure_footer_content.join(", ")}</figcaption>) unless figure_footer_content.empty?

      figure_html = %(<figure>
#{figure_content.convert}
#{figcaption_html}
</figure>)
      create_pass_block parent, figure_html, attrs, subs: nil
    end
  end

  Extensions.register do
    block RichQuoteBlock
  end
end

include Lo48576
