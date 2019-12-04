module Lo48576
  require 'toml-rb'
  require 'asciidoctor/extensions'
  include Asciidoctor

  # `citetitle:[cite title]`
  class CiteTitleInline < Extensions::InlineMacroProcessor
    use_dsl
    named :citetitle
    using_format :short

    def process parent, target, attrs
      %(<cite>#{target}</cite>)
    end
  end

  # `time:time-string[string-to-show]`
  class TimeInline < Extensions::InlineMacroProcessor
    use_dsl
    named :time
    parse_content_as :text

    def process parent, target, attrs
      text = attrs['text']
      text = target if text || text.empty?
      target ||= text
      %(<time datetime="#{target}">#{text}</time>)
    end
  end

  # `q:[quoted-text]` or `q:[lang=language, quoted-text]`
  class QuoteInline < Extensions::InlineMacroProcessor
    use_dsl
    named :q
    parse_content_as :attributes
    format :short

    def process parent, target, attrs
      attrs = (AttributeList.new target).parse if attrs.empty?
      lang = attrs.delete 'lang'
      text = attrs.find{|k, v| k.integer?}[1]
      %(<quote#{%[ xml:lang="#{lang}" lang="#{lang}"] if lang}>#{text}</quote>)
    end
  end

  # ```
  # [caption] **caption** text.
  # ```
  class FigureCaptionBlock < Extensions::BlockProcessor
    use_dsl
    named :figcaption
    on_context [:paragraph, :open, :example]
    content_model :compound

    def process parent, reader, attrs
      doc = parent.document
      if !(doc.basebackend? 'html')
        raise "Unsupported backend #{doc.basebackend}"
      end

      wrapper = create_pass_block parent, nil, attrs, content_model: :compound
      parse_content wrapper, reader.read_lines

      html = %(<figcaption>
#{wrapper.convert}
</figcaption>)
      create_pass_block parent, html, attrs, subs: nil
    end
  end

  # ```
  # [figure]
  # ====
  # foo
  #
  # bar
  #
  # [caption] **caption** text.
  # ====
  # ```
  class FigureBlock < Extensions::BlockProcessor
    use_dsl
    named :figure
    on_context [:paragraph, :open, :example]
    content_model :compound

    def process parent, reader, attrs
      doc = parent.document
      if !(doc.basebackend? 'html')
        raise "Unsupported backend #{doc.basebackend}"
      end

      wrapper = create_pass_block parent, nil, attrs, content_model: :compound
      parse_content wrapper, reader.read_lines

      html = %(<figure>
#{wrapper.convert}
</figure>)
      create_pass_block parent, html, attrs, subs: nil
    end
  end

  # ```
  # quotecaption::uri[asciidoc text caption]
  # ```
  #
  # `uri` part is optional.
  class QuoteCaptionBlockMacro < Extensions::BlockMacroProcessor
    use_dsl
    named :quotecaption
    parse_content_as :compound

    def process parent, target, attrs
      #text = attrs['text']
      #lang = target.split(',').find{|entry| entry.start_with?('lang=')}&.slice(5..)
      #%(<quote#{%[ xml:lang="#{lang}" lang="#{lang}"] if lang}>#{text}</quote>)
      #create_paragraph parent, (attrs.delete 'text'), attrs, subs: nil
      nil
    end
  end

  class QuoteCaptionDetector < Extensions::TreeProcessor
    def process doc
      (doc.find_by context: :quote).each do |quote|
        #quote.blocks.each do |block|
        #  #p block.content
        #end
        #p quote.blocks.find {|block| block.content.start_with? 'quotecaption::'}&.content
      end
    end
  end

  Extensions.register do
    inline_macro CiteTitleInline
    inline_macro QuoteInline
    inline_macro TimeInline
    block FigureCaptionBlock
    block FigureBlock
    block_macro QuoteCaptionBlockMacro
    tree_processor QuoteCaptionDetector
  end
end

include Lo48576
